import '@mantine/core/styles.css';
import {AppShell, MantineProvider} from '@mantine/core';
import HomePage from "./page/HomePage.tsx";

export default function App() {
    return <MantineProvider>
        <AppShell padding="xl" pt={100}>
            <AppShell.Main>
                <HomePage/>
            </AppShell.Main>
        </AppShell>
    </MantineProvider>
}