import {Button, Group, Paper, Stack, Text, TextInput} from "@mantine/core";
import {useRef, useState} from "react";
import {invoke} from "@tauri-apps/api";

export default function HomePage() {
    const [result, setResult] = useState('');
    const input = useRef<HTMLInputElement>(null);

    async function handleClickRender() {
        console.log(input.current?.value)
        const folderStructure = await invoke('get_folder_structure', {path: input.current?.value});
        setResult(folderStructure as string)
    }

    return <Stack>
        <Group>
            <TextInput placeholder="Text" ref={input}/>
            <Button onClick={handleClickRender}>Render</Button>
        </Group>
        <Paper shadow="xs" radius="lg" p="xl" withBorder>
            <Text>{result}</Text>
        </Paper>
    </Stack>
}