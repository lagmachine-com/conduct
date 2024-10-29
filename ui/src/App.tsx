import { createResource, type Component, Show, Switch, Match } from 'solid-js';

import { get } from './api';

import { Button } from './components/ui/button';
import { ColorModeProvider } from '@kobalte/core/color-mode';
import { Separator } from './components/ui/separator';

const fetchProjectInfo = async () => {
  const response = await get("api/json");
  return response.json();
}

const App: Component = () => {
  const [info] = createResource(fetchProjectInfo);

  return (
    <ColorModeProvider initialColorMode="system" >
      <Show when={info()}>
        <div class='border-spacing-10 p-3'>
          <h4 class="text-lg font-medium leading-none">{info().name}</h4>
          <p class="text-sm text-muted-foreground">{info().id}</p>
          <Separator class='my-4'></Separator>
          <div class='flex space-x-3'>
            <Button>Button 1</Button>
            <Button>Button 2</Button>
            <Button>Button 3</Button>
          </div>
        </div>
      </Show>
    </ColorModeProvider>
  );
};


export default App;
