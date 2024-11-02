import { createResource, type Component, Show, Switch, Match } from 'solid-js';

import { get, getSummary, doExport, doCreate, exitDialog } from '../../api';

import { Button } from '../../components/ui/button';
import { ColorModeProvider } from '@kobalte/core/color-mode';
import { Separator } from '../../components/ui/separator';
import { SummaryResponse } from '../../bindings/summary_response';

const DialogCreateSetup: Component = () => {

    return (
        <ColorModeProvider initialColorMode="system" >
            <div class='border-spacing-10 p-3'>
                <div class='flex space-x-3'>
                    <Button on:click={() => exitDialog()}>Done!</Button>
                </div>
            </div>
        </ColorModeProvider>
    );
};


export default DialogCreateSetup;
