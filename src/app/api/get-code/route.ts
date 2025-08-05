import { NextRequest } from 'next/server';
import fs from 'fs';
import path from 'path';

export async function GET(request: NextRequest) {
  const searchParams = request.nextUrl.searchParams;
  const file = searchParams.get('file');

  console.log('API called with file:', file);

  if (!file) {
    return new Response('File parameter is required', { status: 400 });
  }

  try {
    // Security: Only allow files from example_code directory
    const safePath = path.normalize(file);
    console.log('Safe path:', safePath);
    
    if (!safePath.startsWith('example_code/')) {
      console.log('Access denied for path:', safePath);
      return new Response('Access denied', { status: 403 });
    }

    const filePath = path.join(process.cwd(), safePath);
    console.log('Full file path:', filePath);
    console.log('File exists:', fs.existsSync(filePath));
    
    if (!fs.existsSync(filePath)) {
      console.log('File not found:', filePath);
      return new Response(`File not found: ${filePath}`, { status: 404 });
    }

    const fileContent = fs.readFileSync(filePath, 'utf8');
    console.log('File read successfully, length:', fileContent.length);
    
    return new Response(fileContent, {
      headers: {
        'Content-Type': 'text/plain',
        'Cache-Control': 'public, max-age=3600',
      },
    });
  } catch (error) {
    console.error('Error reading file:', error);
    return new Response(`Internal server error: ${error}`, { status: 500 });
  }
}