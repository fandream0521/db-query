import React, { useRef } from 'react';
import Editor, { Monaco } from '@monaco-editor/react';

interface SQLEditorProps {
  value: string;
  onChange: (value: string | undefined) => void;
  height?: string;
  onExecute?: () => void;
}

const SQLEditor: React.FC<SQLEditorProps> = ({ value, onChange, height = '300px', onExecute }) => {
  const editorRef = useRef<any>(null);

  const handleEditorDidMount = (editor: any, monaco: Monaco) => {
    editorRef.current = editor;
    
    // Add Ctrl+Enter / Cmd+Enter shortcut
    editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter, () => {
      if (onExecute) {
        onExecute();
      }
    });
  };

  return (
    <Editor
      height={height}
      defaultLanguage="sql"
      value={value}
      onChange={onChange}
      onMount={handleEditorDidMount}
      theme="vs-dark"
      options={{
        minimap: { enabled: false },
        fontSize: 14,
        wordWrap: 'on',
        automaticLayout: true,
        scrollBeyondLastLine: false,
      }}
    />
  );
};

export default SQLEditor;

