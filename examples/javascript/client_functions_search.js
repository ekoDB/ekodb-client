/**
 * Search Scripts Example - Basic Search Operations
 * 
 * Demonstrates simple search and query operations using scripts
 */

const { EkoDBClient, Stage } = require('@ekodb/ekodb-client');
const dotenv = require('dotenv');

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || 'http://localhost:8080';
const API_KEY = process.env.API_BASE_KEY || 'a-test-api-key-from-ekodb';

async function setupTestData(client) {
  console.log('📋 Setting up test data...');
  
  try {
    await client.deleteCollection('search_docs');
  } catch (e) {
    // Collection doesn't exist, that's fine
  }
  
  const documents = [
    {
      title: 'Introduction to Machine Learning',
      content: 'Machine learning is a subset of artificial intelligence that focuses on building systems that learn from data.',
      category: 'AI',
      tags: ['ml', 'ai', 'data science']
    },
    {
      title: 'Database Design Principles',
      content: 'Good database design involves normalization, indexing, and understanding query patterns.',
      category: 'Database',
      tags: ['database', 'design', 'sql']
    },
    {
      title: 'Vector Databases Explained',
      content: 'Vector databases store and retrieve data based on semantic similarity using embeddings.',
      category: 'Database',
      tags: ['vector', 'database', 'embeddings']
    },
    {
      title: 'Natural Language Processing',
      content: 'NLP enables computers to understand, interpret, and generate human language.',
      category: 'AI',
      tags: ['nlp', 'ai', 'text']
    },
    {
      title: 'Getting Started with ekoDB',
      content: 'ekoDB is a high-performance database with AI capabilities and built-in search.',
      category: 'Database',
      tags: ['ekodb', 'database', 'tutorial']
    },
  ];
  
  for (const doc of documents) {
    await client.insert('search_docs', doc);
  }
  
  console.log(`✅ Inserted ${documents.length} documents\n`);
}

async function listAllDocsScript(client) {
  console.log('📝 Example 1: List All Documents\n');
  
  const script = {
    label: 'list_all_docs',
    name: 'List All Documents',
    version: '1.0',
    parameters: {},
    functions: [Stage.findAll('search_docs')],
    tags: ['search', 'list'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Script saved');
  
  const result = await client.callScript('list_all_docs');
  console.log(`📊 Found ${result.records.length} documents`);
  result.records.forEach((doc, i) => {
    console.log(`   ${i + 1}. ${doc.title?.value || doc.title} (${doc.category?.value || doc.category})`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return scriptId;
}

async function groupByCategoryScript(client) {
  console.log('📝 Example 2: Count Documents by Category\n');
  
  const script = {
    label: 'docs_by_category',
    name: 'Documents by Category',
    version: '1.0',
    parameters: {},
    functions: [
      Stage.findAll('search_docs'),
      Stage.group(['category'], [
        { output_field: 'count', operation: 'Count' },
      ]),
    ],
    tags: ['search', 'analytics'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Script saved');
  
  const result = await client.callScript('docs_by_category');
  console.log(`📊 Documents by category:`);
  result.records.forEach((cat) => {
    console.log(`   ${cat.category?.value || cat.category}: ${cat.count?.value || cat.count} documents`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return scriptId;
}

async function selectFieldsScript(client) {
  console.log('📝 Example 3: Select Specific Fields\n');
  
  const script = {
    label: 'doc_titles',
    name: 'Document Titles',
    version: '1.0',
    parameters: {},
    functions: [
      Stage.findAll('search_docs'),
      Stage.project(['title', 'category'], false),
    ],
    tags: ['search', 'projection'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Script saved');
  
  const result = await client.callScript('doc_titles');
  console.log(`📊 Document titles (${result.records.length} docs):`);
  result.records.forEach((doc, i) => {
    console.log(`   ${i + 1}. ${doc.title?.value || doc.title}`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return scriptId;
}

async function projectDocsScript(client) {
  console.log('📝 Example 4: Project Document Fields\n');
  
  const script = {
    label: 'project_docs',
    name: 'Project Document Fields',
    version: '1.0',
    parameters: {},
    functions: [
      Stage.findAll('search_docs'),
      Stage.project(['title', 'category'], false),
    ],
    tags: ['search', 'projection'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Script saved');
  
  const result = await client.callScript('project_docs');
  console.log(`📊 Projected documents (showing first 3):`);
  result.records.slice(0, 3).forEach((doc, i) => {
    console.log(`   ${i + 1}. ${doc.title?.value || doc.title}`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return scriptId;
}

async function allFieldsScript(client) {
  console.log('📝 Example 5: All Document Fields\n');
  
  const script = {
    label: 'all_docs',
    name: 'All Document Fields',
    version: '1.0',
    parameters: {},
    functions: [
      Stage.findAll('search_docs'),
    ],
    tags: ['search', 'all'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Script saved');
  
  const result = await client.callScript('all_docs');
  console.log(`📊 All documents (${result.records.length} total, showing first 2):`);
  result.records.slice(0, 2).forEach((doc, i) => {
    console.log(`   ${i + 1}. ${doc.title?.value || doc.title} (${doc.category?.value || doc.category})`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return scriptId;
}

async function cleanup(client, scriptIds) {
  console.log('🧹 Cleaning up...');
  
  try {
    for (const id of scriptIds) {
      await client.deleteScript(id);
    }
    await client.deleteCollection('search_docs');
    console.log('✅ Cleanup complete\n');
  } catch (error) {
    console.log('⚠️  Cleanup had some errors (this is okay)\n');
  }
}

async function main() {
  console.log('🚀 ekoDB Search Scripts Example\n');
  
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  
  try {
    await setupTestData(client);
    const scriptIds = [];
    scriptIds.push(await listAllDocsScript(client));
    scriptIds.push(await groupByCategoryScript(client));
    scriptIds.push(await selectFieldsScript(client));
    scriptIds.push(await projectDocsScript(client));
    scriptIds.push(await allFieldsScript(client));
    await cleanup(client, scriptIds);
    
    console.log('✅ All search script examples completed!');
  } catch (error) {
    console.error('❌ Error:', error.message);
    process.exit(1);
  }
}

main();
