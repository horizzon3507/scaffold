function greet(name: string): string {
  return `hello from ${name}`;
}

if (import.meta.main) {
  console.log(greet("{{name}}"));
}
