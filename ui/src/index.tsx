/* @refresh reload */
import { render } from 'solid-js/web';

import './app.css';
import App from './App';
import { Route, Router } from '@solidjs/router';
import DialogCreateSetup from './pages/dialogs/create_setup';

const root = document.getElementById('root');

if (!(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}

render(() => (<Router>
  <Route path="/" component={App} />
  <Route path="/dialogs/create_setup" component={DialogCreateSetup} />
</Router>), root!);
