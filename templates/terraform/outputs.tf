output "project" {
  value = null_resource.{{name_snake}}.triggers.name
}
