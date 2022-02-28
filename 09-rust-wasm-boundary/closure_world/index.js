import("./closure_world").then(({Point}) => {
  const p1 = Point.new(13, 10);
  console.log(p1.distance((x, y) => x - y));
});
