import '@mantine/core/styles.css';
import {AppShell, MantineProvider} from '@mantine/core';
import HomePage from "./page/HomePage.tsx";

export default function App() {
    return <MantineProvider>
        <AppShell header={{height: 150}} navbar={{
            width: 300,
            breakpoint: 'sm'
        }} padding="md">
            <AppShell.Header p="md">
            </AppShell.Header>
            <AppShell.Navbar p="md">
            </AppShell.Navbar>
            <AppShell.Main>
                <HomePage/>
            </AppShell.Main>
        </AppShell>
    </MantineProvider>
}