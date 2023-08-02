provider "aws" {
  region = "us-east-2"
}

data "aws_vpc" "default" {
  default = true
}

resource "aws_subnet" "fargate_subnet" {
  vpc_id            = data.aws_vpc.default.id
  cidr_block        = "172.31.48.0/20"
  availability_zone = "us-east-2a"

  tags = {
    Name = "fargate_subnet"
  }
}

locals {
  subnet_ids = [aws_subnet.fargate_subnet.id]
}

resource "aws_ecs_cluster" "cluster" {
  name = "fargate-cluster"
}

resource "aws_ecs_task_definition" "task" {
  family                   = "service-task"
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = "256"
  memory                   = "512"
  execution_role_arn       = aws_iam_role.ecs_role.arn

  container_definitions = <<DEFINITION
  [
    {
      "name": "celestia_bridge_service",
      "image": "dubbelosix/sov-celestia-local",
      "essential": true,
      "portMappings": [
        {
          "containerPort": 26657,
          "hostPort": 26657,
          "protocol": "tcp"
        }, 
        {
          "containerPort": 26659,
          "hostPort": 26659,
          "protocol": "tcp"
        }
      ]
    },
    {
      "name": "sov_monovm",
      "image": "public.ecr.aws/c4i6k4r8/sov-monovm",
      "essential": true,
      "portMappings": [
        {
          "containerPort": 12345,
          "hostPort": 12345,
          "protocol": "tcp"
        }
      ]
    }
  ]
  DEFINITION
}

resource "aws_ecs_service" "service" {
  name            = "fargate-service"
  cluster         = aws_ecs_cluster.cluster.id
  task_definition = aws_ecs_task_definition.task.arn
  desired_count   = 1
  launch_type     = "FARGATE"

  network_configuration {
    assign_public_ip = false
    subnets          = local.subnet_ids
    security_groups  = [aws_security_group.sg.id]
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.target_group.arn
    container_name   = "sov_monovm"
    container_port   = 8000
  }
}

resource "aws_iam_role" "ecs_role" {
  name = "ecs_role"
  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "ecs-tasks.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
EOF
}

resource "aws_security_group" "sg" {
  name        = "fargate_sg"
  description = "Fargate Security Group"
  vpc_id      = data.aws_vpc.default.id

  ingress {
    from_port   = 12345
    to_port     = 12345
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 8000
    to_port     = 8000
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_acm_certificate" "cert" {
  domain_name       = "testnet.sov-monovm.mvlabs.net"
  validation_method = "DNS"
}

resource "aws_route53_record" "cert_validation" {
  for_each = {
    for dvo in aws_acm_certificate.cert.domain_validation_options : dvo.domain_name => {
      name   = dvo.resource_record_name
      record = dvo.resource_record_value
      type   = dvo.resource_record_type
    }
  }

  name    = each.value.name
  type    = each.value.type
  zone_id = "Z02153823HW9V0MX2AVRG"
  records = [each.value.record]
  ttl     = 60
}

resource "aws_acm_certificate_validation" "cert" {
  certificate_arn         = aws_acm_certificate.cert.arn
  validation_record_fqdns = [for record in aws_route53_record.cert_validation : record.fqdn]
}

resource "aws_lb" "lb" {
  name               = "fargate-lb"
  internal           = false
  load_balancer_type = "application"
  security_groups    = [aws_security_group.lb_sg.id]
  subnets            = local.subnet_ids
}

resource "aws_lb_listener" "listener" {
  load_balancer_arn = aws_lb.lb.arn
  port              = "443"
  protocol          = "HTTPS"
  ssl_policy        = "ELBSecurityPolicy-2016-08"
  certificate_arn   = aws_acm_certificate_validation.cert.certificate_arn

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.target_group.arn
  }
}


resource "aws_lb_target_group" "target_group" {
  name     = "fargate-tg"
  port     = 8000
  protocol = "HTTP"
  vpc_id   = data.aws_vpc.default.id

  health_check {
    enabled             = true
    interval            = 30
    path                = "/"
    timeout             = 3
    healthy_threshold   = 3
    unhealthy_threshold = 3
  }
}

resource "aws_security_group" "lb_sg" {
  name        = "lb_sg"
  description = "LB security group"
  vpc_id      = data.aws_vpc.default.id

  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}