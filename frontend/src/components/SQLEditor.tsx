import React from 'react';
import Editor from '@monaco-editor/react';

interface SQLEditorProps {
  value: string;
  onChange: (value: string | undefined) => void;
  height?: string;
}

const SQLEditor: React.FC<SQLEditorProps> = ({ value, onChange, height = '300px' }) => {
  return (
    <Editor
      height={height}
      defaultLanguage="sql"
      value={value}
      onChange={onChange}
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

