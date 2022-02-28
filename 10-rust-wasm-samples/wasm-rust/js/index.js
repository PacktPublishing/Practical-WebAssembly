const rust = import('../pkg/index.js');
rust.then(module => {
    console.log(module.is_palindrome('tattarrattat')); // returns true
});
