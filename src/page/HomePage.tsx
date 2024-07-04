import {ActionIcon, Button, CopyButton, Flex, Group, Paper, rem, Stack, TextInput, Tooltip} from "@mantine/core";
import {useRef, useState} from "react";
import {invoke} from "@tauri-apps/api";
import Markdown from "react-markdown";
import {IconCheck, IconCopy} from '@tabler/icons-react';

export default function HomePage() {
    const [result, setResult] = useState('');
    const [textToCopy, setTextToCopy] = useState('');
    const input = useRef<HTMLInputElement>(null);


    async function handleClickRender() {
        const folderStructure = await invoke('get_folder_structure', {path: input.current?.value});
        setResult('```\n' + folderStructure as string + '\n```')
        setTextToCopy('\nI will describe my project to you. Confirm with me that you clearly understand it.\nJust answer yes or no. I don\'t need any more information.' + folderStructure as string)
    }

    return <Stack>
        <Group justify="center">
            <TextInput placeholder="Enter your path" ref={input} variant="filled" w={700} size="lg" radius='xl'/>
            <Button onClick={handleClickRender} radius='xl' variant="gradient"
                    gradient={{from: 'violet', to: 'pink', deg: 91}} size='lg'>Render</Button>
        </Group>
        <Paper shadow="xs" radius="lg" p="md" withBorder mt={30}>
            <Flex justify="space-between">
                <Stack>
                    <Markdown>
                        {result}
                    </Markdown>
                </Stack>
                <CopyButton value={textToCopy} timeout={2000}>
                    {({copied, copy}) => (
                        <Tooltip label={copied ? 'Copied' : 'Copy'} withArrow position="right">
                            <ActionIcon color={copied ? 'teal' : 'gray'} variant="subtle" onClick={copy}>
                                {copied ? (
                                    <IconCheck style={{width: rem(16)}}/>
                                ) : (
                                    <IconCopy style={{width: rem(16)}}/>
                                )}
                            </ActionIcon>
                        </Tooltip>
                    )}
                </CopyButton>
            </Flex>
        </Paper>
    </Stack>
}