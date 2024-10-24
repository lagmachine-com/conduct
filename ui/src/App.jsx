import logo from './logo.svg';
import styles from './App.module.css';
import { createSignal, createResource, Switch, Match, Suspense } from "solid-js";
import { get } from './api'

const fetchProjectInfo = async () => {
  const response = await get("api/json");
  return response.text();
}

function App() {
  const [info] = createResource(fetchProjectInfo);

  return (
    <div class={styles.App}>
      <header class={styles.header}>
        <Show when={info.loading}>
          <p>Loading...</p>
        </Show>
        <Switch>
          <Match when={info.error}>
            <span>Error: {info.error}</span>
          </Match>
          <Match when={info()}>
            <div>{info()}</div>
          </Match>
        </Switch>
      </header>
    </div>
  );
}

export default App;
