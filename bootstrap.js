import './static/style.scss';
import './static/tailwind.css';

import("./pkg").then(module => {
  module.run_app();
});
