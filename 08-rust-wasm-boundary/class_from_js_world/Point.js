export class Point {
  constructor(x, y) {
      this.x = x;
      this.y = y;
  }

  get_x() {
      return this.x;
  }

  get_y() {
      return this.y;
  }

  set_x(x) {
      this.x = x;
  }

  set_y(y) {
      this.y = y;
  }

  add(p1) {
      this.x += p1.x;
      this.y += p1.y;
  }
}