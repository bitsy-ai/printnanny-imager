function truncate(str: string): string {
  return str.slice(0, 10) + "..." + str.slice(str.length - 10, str.length);
}

export { truncate };
