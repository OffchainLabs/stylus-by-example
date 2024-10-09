import type { MDXComponents } from 'mdx/types';
import { css } from '@/styled-system/css';
import { styled } from '@/styled-system/jsx';
import { DetailedHTMLProps, HTMLAttributes, ReactPropTypes } from 'react';
import { CodePanel } from '@/components/app/code_panel';

function Pre({ children }: DetailedHTMLProps<HTMLAttributes<HTMLPreElement>, HTMLPreElement>) {
  return <>{children}</>;
}

interface CodeProps extends React.HtmlHTMLAttributes<HTMLDivElement> {}

function Code({ children, className }: CodeProps) {
  if (className?.startsWith('language-')) {
    className = className.split('-').pop();
    return <CodePanel language={className}>{`${children}`}</CodePanel>;
  } else {
    return (
      <code
        className={css({
          bg: { _light: 'stone.200', _dark: `stone.700` },
          p: '2px',
          borderRadius: 'sm',
        })}
      >
        {children}
      </code>
    );
  }
}

export function useMDXComponents(components: MDXComponents): MDXComponents {
  return {
    ...components,

    h1: ({ children }) => (
      <h1
        className={css({
          fontFamily: 'heading',
          fontSize: '3xl',
          fontWeight: 'bold',
          mt: '4',
          mb: '2',
        })}
      >
        {children}
      </h1>
    ),
    h2: ({ children }) => (
      <h2
        className={css({
          fontFamily: 'heading',
          fontSize: '2xl',
          fontWeight: 'bold',
          mt: '5',
          mb: '2',
        })}
      >
        {children}
      </h2>
    ),
    h3: ({ children }) => (
      <h3
        className={css({
          fontFamily: 'mono',
          mt: '5',
          mb: '2',
          fontSize: '16px',
        })}
      >
        {children}
      </h3>
    ),
    em: ({ children }) => <em className={css({ fontStyle: 'italic' })}>{children}</em>,
    p: ({ children }) => (
      <p
        className={css({
          fontSize: '16px',
          mt: '3',
          mb: '3',
          lineHeight: '1.75',
          fontFamily: 'body',
        })}
      >
        {children}
      </p>
    ),
    ul: ({ children }) => (
      <styled.ul
        listStyle="inside"
        fontFamily="body"
        fontSize="16px"
        ml="0"
        pl="1em"
        pr="-1em"
        lineHeight="2"
      >
        {children}
      </styled.ul>
    ),
    a: ({ children, href }) => {
      return (
        <styled.a textDecoration="none" target="_blank" cursor="pointer" href={href}>
          {children}
        </styled.a>
      );
    },
    pre: Pre,
    code: Code,
  };
}
