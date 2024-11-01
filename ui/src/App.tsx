import { createResource, type Component, Show, Switch, Match } from 'solid-js';

import { get, getSummary, doExport, doCreate } from './api';

import { Button } from './components/ui/button';
import { ColorModeProvider } from '@kobalte/core/color-mode';
import { Separator } from './components/ui/separator';
import { SummaryResponse } from './bindings/summary_response';

const App: Component = () => {
  const [info] = createResource(getSummary);

  return (
    <ColorModeProvider initialColorMode="system" >
      <Show when={info()}>
        <div class='border-spacing-10 p-3'>
          <h4 class="text-lg font-medium leading-none">{info()!.display_name}</h4>
          <p class="text-sm text-muted-foreground">{info()!.identifier}</p>
          <Separator class='my-4'></Separator>
          <div class='flex space-x-3'>
            <Button on: click={() => doExport()}>Export Asset</Button>
            <Button on: click={() => doCreate()}>Create Asset</Button>
            <Button>Button 3</Button>
          </div>
        </div>
      </Show>
    </ColorModeProvider>
  );
};


export default App;
