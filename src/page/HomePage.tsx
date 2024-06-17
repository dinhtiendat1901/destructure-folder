import {Button, Group, Paper, Stack, TextInput} from "@mantine/core";
import {useRef, useState} from "react";
import {invoke} from "@tauri-apps/api";
import Markdown from "react-markdown";

export default function HomePage() {
    const [result, setResult] = useState('');
    const input = useRef<HTMLInputElement>(null);

    async function handleClickRender() {
        console.log(input.current?.value)
        const folderStructure = await invoke('get_folder_structure', {path: input.current?.value});
        setResult('```\n' + folderStructure as string + '\n```')
    }

    return <Stack>
        <Group>
            <TextInput placeholder="Text" ref={input}/>
            <Button onClick={handleClickRender}>Render</Button>
        </Group>
        <Paper shadow="xs" radius="lg" p="xl" withBorder>
            <Markdown>
                {result}
            </Markdown>
        </Paper>
    </Stack>
}