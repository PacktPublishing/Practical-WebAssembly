import("./jsapi").then(module => {
  let m = module.new_js_map();
  m.set("Hi", "Hi");

  console.log(m); // prints Map { "Hi" ->  "Hi" }

  console.log(module.set_get_js_map());  // prints "bar"

  console.log(module.run_through_map()); // prints 15
});