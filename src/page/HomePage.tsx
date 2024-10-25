import {ActionIcon, Button, Code, CopyButton, Group, Paper, rem, Stack, Text, Tooltip} from "@mantine/core";
import {useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import Markdown from "react-markdown";
import {IconCheck, IconCopy} from '@tabler/icons-react';
import {open} from '@tauri-apps/plugin-dialog';

interface FileInfo {
    path: string,
    content: string
}

export default function HomePage() {
    const [tree, setTree] = useState('');
    const [listFile, setListFile] = useState<FileInfo[]>([]);
    const [path, setPath] = useState('');
    const [textCopy, setTextCopy] = useState('');

    async function handleClickRender() {
        const result = await invoke<[string, string, FileInfo[]]>('get_folder_structure', {path: path});
        setTree('```\n' + result[0] + '\n```')
        setListFile(result[2])


        setTextCopy('\nI will describe my project to you. Confirm with me that you clearly understand it.\nJust answer yes or no. I don\'t need any more information.\n' + result[0] + result[1])
    }


    const listFileRender = listFile.map(fileInfo => (<div key={fileInfo.path}>
            <Text fw={700}>Content of file in path {fileInfo.path}:</Text>
            <Code block>{fileInfo.content}</Code>
        </div>
    ))


    async function handleSelectFolder() {
        const selectedPath = await open({
            directory: true
        });
        setPath(selectedPath as string)
    }


    return <Stack>
        <Group justify="center">
            <Text w={700} size="xl" variant="gradient"
                  gradient={{from: 'blue', to: 'cyan', deg: 90}}>{path}</Text>
            <Button onClick={handleSelectFolder} radius='xl' variant="gradient"
                    gradient={{from: 'violet', to: 'pink', deg: 91}} size='lg'>Select Folder</Button>
            <Button onClick={handleClickRender} radius='xl' variant="gradient"
                    gradient={{from: 'violet', to: 'pink', deg: 91}} size='lg'>Render</Button>
        </Group>
        <Paper shadow="xs" radius="lg" p="md" withBorder mt={30}>
            <Stack>
                <CopyButton value={textCopy} timeout={7000}>
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
                <Markdown>
                    {tree}
                </Markdown>
                {listFileRender}
            </Stack>
        </Paper>
    </Stack>
}