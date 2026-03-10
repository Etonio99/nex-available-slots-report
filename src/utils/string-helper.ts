const snakeCaseToTitleCase = (input: string) => {
  return input
    .split('_')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
};

export { snakeCaseToTitleCase };
