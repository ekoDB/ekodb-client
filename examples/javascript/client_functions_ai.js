/**
 * AI Scripts Example - Chat and Embed Operations
 * 
 * Demonstrates AI operations in scripts:
 * - Chat completions with context
 * - Embedding generation
 * - Simple AI workflows
 */

const { EkoDBClient, Stage, ChatMessage } = require('@ekodb/ekodb-client');
const dotenv = require('dotenv');

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || 'http://localhost:8080';
const API_KEY = process.env.API_BASE_KEY || 'a-test-api-key-from-ekodb';

async function setupTestData(client) {
  console.log('📋 Setting up test data...');
  
  const articles = [
    {
      title: 'Getting Started with ekoDB',
      content: 'ekoDB is a high-performance database with AI capabilities...',
      status: 'published',
    },
    {
      title: 'Advanced Query Patterns',
      content: 'Learn how to build complex queries in ekoDB...',
      status: 'published',
    },
    {
      title: 'Draft Article',
      content: 'This is a draft that needs review...',
      status: 'draft',
    }
  ];
  
  for (const article of articles) {
    await client.insert('ai_articles', article);
  }
  
  console.log(`✅ Created ${articles.length} articles\n`);
}

async function chatCompletionScript(client) {
  console.log('📝 Example 1: Simple Chat Completion\n');
  
  const script = {
    label: 'ai_assistant',
    name: 'AI Chat Assistant',
    description: 'Simple AI chat completion',
    version: '1.0',
    parameters: {},
    functions: [
      Stage.chat(
        [
          ChatMessage.system('You are a helpful database assistant. Be concise and technical.'),
          ChatMessage.user('What are the benefits of using vector databases?')
        ],
        'gpt-4',
        0.7
      )
    ],
    tags: ['ai', 'chat'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Chat script saved');
  
  const result = await client.callScript('ai_assistant');
  
  console.log('🤖 AI Response:');
  console.log(`   ${result.records[0]?.response?.value || result.records[0]?.response || 'No response'}`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return scriptId;
}

async function embedGenerationScript(client) {
  console.log('📝 Example 2: Generate Embeddings\n');
  
  const script = {
    label: 'generate_embeddings',
    name: 'Generate Article Embeddings',
    description: 'Generate embeddings for all articles',
    version: '1.0',
    parameters: {},
    functions: [
      Stage.findAll('ai_articles'),
      Stage.embed('content', 'embedding', 'text-embedding-ada-002')
    ],
    tags: ['ai', 'embeddings'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Embedding script saved');
  
  const result = await client.callScript('generate_embeddings');
  
  console.log(`📊 Generated embeddings for ${result.records.length} articles`);
  result.records.forEach((article, i) => {
    const embeddingSize = article.embedding?.length || 0;
    console.log(`   ${i + 1}. "${article.title?.value || article.title}" - ${embeddingSize}D vector`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return scriptId;
}

async function listArticlesScript(client) {
  console.log('📝 Example 3: List All Articles\n');
  
  const script = {
    label: 'list_articles',
    name: 'List All Articles',
    version: '1.0',
    parameters: {},
    functions: [Stage.findAll('ai_articles')],
    tags: ['articles'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Script saved');
  
  const result = await client.callScript('list_articles');
  console.log(`📊 Found ${result.records.length} articles`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return scriptId;
}

async function cleanup(client, scriptIds) {
  console.log('🧹 Cleaning up...');
  
  try {
    for (const id of scriptIds) {
      await client.deleteScript(id);
    }
    await client.deleteCollection('ai_articles');
    console.log('✅ Cleanup complete\n');
  } catch (error) {
    console.log('⚠️  Cleanup had some errors (this is okay)\n');
  }
}

async function main() {
  console.log('🚀 ekoDB AI Scripts Example\n');
  console.log('⚠️  Note: These examples require AI API credentials (OpenAI, etc.)\n');
  
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  
  try {
    await setupTestData(client);
    const scriptIds = [];
    scriptIds.push(await chatCompletionScript(client));
    scriptIds.push(await embedGenerationScript(client));
    scriptIds.push(await listArticlesScript(client));
    await cleanup(client, scriptIds);
    
    console.log('✅ All AI examples completed!');
  } catch (error) {
    console.error('❌ Error:', error.message);
    if (error.message.includes('AI') || error.message.includes('model')) {
      console.log('\n💡 Tip: Make sure AI features are configured in your ekoDB server');
    }
    process.exit(1);
  }
}

main();
