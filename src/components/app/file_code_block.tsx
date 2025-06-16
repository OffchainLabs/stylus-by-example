'use client';

import { useEffect, useState } from 'react';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { dracula, prism } from 'react-syntax-highlighter/dist/esm/styles/prism';
import { css } from '@/styled-system/css';
import { CopyButton } from '@/components/app/copy_button';

interface FileCodeBlockProps {
  filePath: string;
  language?: string;
  title?: string;
}

export function FileCodeBlock({ filePath, language = 'rust', title }: FileCodeBlockProps) {
  const [code, setCode] = useState<string>('Loading...');
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function fetchCode() {
      try {
        setIsLoading(true);
        const response = await fetch(`/api/get-code?file=${encodeURIComponent(filePath)}`);
        if (!response.ok) {
          throw new Error(`Failed to fetch: ${response.statusText}`);
        }
        const fileContent = await response.text();
        setCode(fileContent);
        setError(null);
      } catch (err) {
        setError(`Error loading file: ${err instanceof Error ? err.message : 'Unknown error'}`);
        setCode('// Error loading file');
      } finally {
        setIsLoading(false);
      }
    }

    fetchCode();
  }, [filePath]);

  if (error) {
    return (
      <div className="my-4">
        {title && (
          <h3 className="text-lg font-semibold mb-2 text-stone-900 dark:text-stone-100">
            {title}
          </h3>
        )}
        <div className="bg-red-50 border border-red-200 p-4 rounded dark:bg-red-900/20 dark:border-red-800">
          <p className="text-red-600 dark:text-red-400">{error}</p>
        </div>
      </div>
    );
  }

  const content = code.trim();

  return (
    <div className="my-4">
      {title && (
        <h3 className="text-lg font-semibold mb-2 text-stone-900 dark:text-stone-100">
          {title}
        </h3>
      )}
      
      {/* Dark mode version */}
      <div
        className={css({
          borderColor: 'stone.900',
          borderWidth: '1px',
          borderRadius: 'lg',
          overflow: 'hidden',
          maxW: '4xl',
          display: { _light: 'none' },
          position: 'relative',
        })}
      >
        {!isLoading && <CopyButton content={content} />}
        <SyntaxHighlighter
          language={language}
          style={dracula}
          customStyle={{ margin: 0 }}
          codeTagProps={{
            style: { fontFamily: 'var(--font-mono)', fontSize: '12px' },
          }}
          wrapLines={true}
          showLineNumbers={true}
        >
          {content}
        </SyntaxHighlighter>
      </div>

      {/* Light mode version */}
      <div
        className={css({
          borderColor: { _light: 'stone.200', _dark: 'stone.700' },
          borderWidth: '1px',
          borderRadius: 'lg',
          overflow: 'hidden',
          display: { _dark: 'none' },
          position: 'relative',
        })}
      >
        {!isLoading && <CopyButton content={content} />}
        <SyntaxHighlighter
          language={language}
          style={prism}
          customStyle={{ margin: 0 }}
          codeTagProps={{
            style: { fontFamily: 'var(--font-mono)', fontSize: '12px' },
          }}
          wrapLines={true}
          showLineNumbers={true}
        >
          {content}
        </SyntaxHighlighter>
      </div>
    </div>
  );
}