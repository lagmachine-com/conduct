/* @refresh reload */
import { render } from 'solid-js/web';

import './app.css';
import App from './App';
import { Route, Router, RouteSectionProps } from '@solidjs/router';
import DialogCreateSetup from './pages/dialogs/create_setup';
import DialogLoadAsset from './pages/dialogs/load_asset';
import { ColorModeProvider } from '@kobalte/core/color-mode';
import { Component } from 'solid-js';
import DialogExport from './pages/dialogs/export';
import DialogIngest from './pages/dialogs/ingest';

const root = document.getElementById('root');

if (!(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}

const rootComponent: Component<RouteSectionProps> = (props) => {
  return (
    <ColorModeProvider initialColorMode='dark'>
      {props.children}
    </ColorModeProvider>
  )
}

render(() => (<Router root={rootComponent}>
  <Route path="/" component={App} />
  <Route path="/dialogs/create_setup" component={DialogCreateSetup} />
  <Route path="/dialogs/load_asset" component={DialogLoadAsset} />
  <Route path="/dialogs/export" component={DialogExport} />
  <Route path="/dialogs/ingest" component={DialogIngest} />
</Router>), root!);
