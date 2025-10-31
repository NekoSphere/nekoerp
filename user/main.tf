provider "aws" {
  region = var.aws_region
}

resource "aws_instance" "nekoerp_auth" {
  ami           = var.ami
  instance_type = var.instance_type

  tags = {
    Name = var.name
    Email = var.email
    Uuid = var.uuid
  }
}

variable "ami" { type = string }
variable "name" { type = string }
variable "email" { type = string }
variable "instance_type" { type = string default = "t4g.small" }
variable "aws_region" { type = string default = "sa-east-1" }
variable "uuid" { type = string }
output "instance_id" { value = aws_instance.nekoerp_auth.id }
output "public_ip"   { value = aws_instance.nekoerp_auth.public_ip }

# terraform init
# terraform apply -var="ami=ami-0abcd1234" -var="name=ClienteX" -var="email=cliente@dom.com" -var="uuid=<uuid>" -auto-approve
