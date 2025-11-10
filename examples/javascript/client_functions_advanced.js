/**
 * Advanced Scripts Example - Query, Sort, Limit, Group
 * 
 * Demonstrates advanced query and aggregation operations using simple patterns
 */

const { EkoDBClient, Stage } = require('@ekodb/ekodb-client');
const dotenv = require('dotenv');

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || 'http://localhost:8080';
const API_KEY = process.env.API_BASE_KEY || 'a-test-api-key-from-ekodb';

async function setupTestData(client) {
  console.log('📋 Setting up test data...');
  
  try {
    await client.deleteCollection('advanced_products');
  } catch (e) {
    // Collection doesn't exist, that's fine
  }
  
  const products = [
    { name: 'Laptop Pro', category: 'Electronics', price: 1299, stock: 15, rating: 4.8 },
    { name: 'Wireless Mouse', category: 'Electronics', price: 29, stock: 45, rating: 4.5 },
    { name: 'Desk Chair', category: 'Furniture', price: 349, stock: 8, rating: 4.2 },
    { name: 'Standing Desk', category: 'Furniture', price: 599, stock: 12, rating: 4.7 },
    { name: 'USB-C Cable', category: 'Electronics', price: 19, stock: 100, rating: 4.3 },
    { name: 'Monitor 27"', category: 'Electronics', price: 399, stock: 20, rating: 4.6 },
    { name: 'Bookshelf', category: 'Furniture', price: 149, stock: 5, rating: 4.1 },
    { name: 'Keyboard', category: 'Electronics', price: 89, stock: 30, rating: 4.4 },
    { name: 'Office Lamp', category: 'Furniture', price: 79, stock: 18, rating: 4.05 },
    { name: 'Webcam HD', category: 'Electronics', price: 119, stock: 22, rating: 4.5 },
  ];
  
  for (const product of products) {
    await client.insert('advanced_products', product);
  }
  
  console.log(`✅ Created ${products.length} products\n`);
}

async function listAllScript(client) {
  console.log('📝 Example 1: List All Products\n');
  
  const script = {
    label: 'list_all_products',
    name: 'List All Products',
    version: '1.0',
    parameters: {},
    functions: [Stage.findAll('advanced_products')],
    tags: ['products', 'list'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Script saved');
  
  const result = await client.callScript('list_all_products');
  console.log(`📊 Found ${result.records.length} products`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return scriptId;
}

async function groupByCategoryScript(client) {
  console.log('📝 Example 2: Group Products by Category\n');
  
  const script = {
    label: 'products_by_category',
    name: 'Products by Category',
    version: '1.0',
    parameters: {},
    functions: [
      Stage.findAll('advanced_products'),
      Stage.group(['category'], [
        { output_field: 'count', operation: 'Count' },
        { output_field: 'avg_price', operation: 'Average', input_field: 'price' },
      ]),
    ],
    tags: ['products', 'analytics'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Script saved');
  
  const result = await client.callScript('products_by_category');
  console.log(`📊 Found ${result.records.length} categories`);
  result.records.forEach((cat) => {
    console.log(`   ${cat.category?.value || cat.category}: ${cat.count?.value || cat.count} items (avg $${(cat.avg_price?.value || cat.avg_price).toFixed(2)})`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return scriptId;
}

async function countProductsScript(client) {
  console.log('📝 Example 3: Count All Products\n');
  
  const script = {
    label: 'count_products',
    name: 'Count All Products',
    version: '1.0',
    parameters: {},
    functions: [
      Stage.findAll('advanced_products'),
      Stage.count(),
    ],
    tags: ['products', 'count'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Script saved');
  
  const result = await client.callScript('count_products');
  console.log(`📊 Total products: ${result.records[0]?.count?.value || result.records[0]?.count || 0}`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return scriptId;
}

async function multiStageAggregationScript(client) {
  console.log('📝 Example 4: Multi-Stage Aggregation\n');
  
  const script = {
    label: 'category_analysis',
    name: 'Category Analysis',
    version: '1.0',
    parameters: {},
    functions: [
      Stage.findAll('advanced_products'),
      Stage.group(['category'], [
        { output_field: 'count', operation: 'Count' },
        { output_field: 'total_stock', operation: 'Sum', input_field: 'stock' },
        { output_field: 'avg_rating', operation: 'Average', input_field: 'rating' },
      ]),
    ],
    tags: ['products', 'analytics', 'multi-stage'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Script saved');
  
  const result = await client.callScript('category_analysis');
  console.log(`📊 Category analysis (${result.records.length} categories):`);
  result.records.forEach((cat) => {
    console.log(`   ${cat.category?.value || cat.category}:`);
    console.log(`      Products: ${cat.count?.value || cat.count} | Stock: ${cat.total_stock?.value || cat.total_stock} | Avg Rating: ⭐${(cat.avg_rating?.value || cat.avg_rating).toFixed(2)}`);
  });
  console.log(`\n⏱️  Total execution time: ${result.stats.execution_time_ms}ms`);
  console.log('📈 Pipeline stages:');
  result.stats.stage_stats.forEach((stage, index) => {
    console.log(`   ${index + 1}. ${stage.stage}: ${stage.input_count} → ${stage.output_count} records (${stage.execution_time_ms}ms)`);
  });
  console.log();
  
  return scriptId;
}

async function projectFieldsScript(client) {
  console.log('📝 Example 5: Project Specific Fields\n');
  
  const script = {
    label: 'product_summary',
    name: 'Product Summary',
    version: '1.0',
    parameters: {},
    functions: [
      Stage.findAll('advanced_products'),
      Stage.project(['name', 'price', 'rating'], false),
    ],
    tags: ['products', 'projection'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Script saved');
  
  const result = await client.callScript('product_summary');
  console.log(`📊 Product summaries (${result.records.length} items, showing first 3):`);
  result.records.slice(0, 3).forEach((p, i) => {
    console.log(`   ${i + 1}. ${p.name?.value || p.name} - $${p.price?.value || p.price} (⭐${p.rating?.value || p.rating})`);
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
    await client.deleteCollection('advanced_products');
    console.log('✅ Cleanup complete\n');
  } catch (error) {
    console.log('⚠️  Cleanup had some errors (this is okay)\n');
  }
}

async function main() {
  console.log('🚀 ekoDB Advanced Scripts Example\n');
  
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  
  try {
    await setupTestData(client);
    const scriptIds = [];
    scriptIds.push(await listAllScript(client));
    scriptIds.push(await groupByCategoryScript(client));
    scriptIds.push(await countProductsScript(client));
    scriptIds.push(await multiStageAggregationScript(client));
    scriptIds.push(await projectFieldsScript(client));
    await cleanup(client, scriptIds);
    
    console.log('✅ All advanced script examples completed!');
  } catch (error) {
    console.error('❌ Error:', error.message);
    process.exit(1);
  }
}

main();
