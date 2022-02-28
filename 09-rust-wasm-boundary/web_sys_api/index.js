import("./web_sys_api").then(module => {
  class Pie extends HTMLElement {
      constructor() {
          super();
          let shadow = this.attachShadow({ mode: 'open' });
          let style = document.createElement('style');

          style.textContent = `
                  svg {
                      width:100px;
                      height: 100px;
                      background: yellowgreen;
                      border-radius: 50%;
                  }

                  circle {
                      fill: yellowgreen;
                      stroke: #655;
                      stroke-width: 32;
                  }`;

         shadow.appendChild(module.draw(this.getAttribute('value'));
         shadow.appendChild(style);
     }
 }

  customElements.define('pie-chart', Pie);

  setInterval(() => {
      let r = Math.floor(Math.random() * 100);
      document.getElementsByTagName('body')[0].innerHTML = `
          <pie-chart value='${r}' />`;
  }, 1000);
});
