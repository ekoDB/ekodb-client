/**
 * CRUD Scripts Example - Basic Data Operations
 * 
 * Demonstrates basic CRUD operations using scripts:
 * - FindAll queries
 * - Group aggregations
 * - Simple data transformations
 */

const { EkoDBClient, Stage } = require('@ekodb/ekodb-client');
const dotenv = require('dotenv');

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || 'http://localhost:8080';
const API_KEY = process.env.API_BASE_KEY || 'a-test-api-key-from-ekodb';

async function setupTestData(client) {
  console.log('📋 Setting up test data...');
  
  try {
    await client.deleteCollection('crud_users');
  } catch (e) {
    // Collection doesn't exist, that's fine
  }
  
  const users = [];
  for (let i = 1; i <= 10; i++) {
    const user = await client.insert('crud_users', {
      name: `User ${i}`,
      email: `user${i}@example.com`,
      status: i % 3 === 0 ? 'inactive' : 'active',
      score: i * 10,
      role: i <= 3 ? 'admin' : 'user'
    });
    users.push(user.id);
  }
  
  console.log(`✅ Created ${users.length} test users\n`);
  return users;
}

async function listUsersScript(client) {
  console.log('📝 Example 1: List All Users\n');
  
  const script = {
    label: 'list_all_users',
    name: 'List All Users',
    version: '1.0',
    parameters: {},
    functions: [Stage.findAll('crud_users')],
    tags: ['users', 'list'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Script saved');
  
  const result = await client.callScript('list_all_users');
  console.log(`📊 Found ${result.records.length} users`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return scriptId;
}

async function countByStatusScript(client) {
  console.log('📝 Example 2: Count Users by Status\n');
  
  const script = {
    label: 'users_by_status',
    name: 'Users by Status',
    version: '1.0',
    parameters: {},
    functions: [
      Stage.findAll('crud_users'),
      Stage.group(['status'], [
        { output_field: 'count', operation: 'Count' },
      ]),
    ],
    tags: ['users', 'analytics'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Script saved');
  
  const result = await client.callScript('users_by_status');
  console.log(`📊 User counts by status:`);
  result.records.forEach((group) => {
    console.log(`   ${group.status?.value || group.status}: ${group.count?.value || group.count} users`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return scriptId;
}

async function avgScoreByRoleScript(client) {
  console.log('📝 Example 3: Average Score by Role\n');
  
  const script = {
    label: 'avg_score_by_role',
    name: 'Average Score by Role',
    version: '1.0',
    parameters: {},
    functions: [
      Stage.findAll('crud_users'),
      Stage.group(['role'], [
        { output_field: 'count', operation: 'Count' },
        { output_field: 'avg_score', operation: 'Average', input_field: 'score' },
      ]),
    ],
    tags: ['users', 'analytics'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Script saved');
  
  const result = await client.callScript('avg_score_by_role');
  console.log(`📊 Average scores by role:`);
  result.records.forEach((group) => {
    console.log(`   ${group.role?.value || group.role}: ${(group.avg_score?.value || group.avg_score).toFixed(1)} (${group.count?.value || group.count} users)`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return scriptId;
}

async function topUsersScript(client) {
  console.log('📝 Example 4: Top Users by Score\n');
  
  const script = {
    label: 'top_users',
    name: 'Top Users by Score',
    version: '1.0',
    parameters: {},
    functions: [
      Stage.findAll('crud_users'),
      Stage.project(['name', 'email', 'score'], false),
    ],
    tags: ['users', 'leaderboard'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Script saved');
  
  const result = await client.callScript('top_users');
  console.log(`📊 Users (showing first 5 of ${result.records.length}):`);
  result.records.slice(0, 5).forEach((user, i) => {
    console.log(`   ${i + 1}. ${user.name?.value || user.name} (${user.email?.value || user.email}) - Score: ${user.score?.value || user.score}`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  
  return scriptId;
}

async function userSummaryScript(client) {
  console.log('📝 Example 5: User Summary Statistics\n');
  
  const script = {
    label: 'user_summary',
    name: 'User Summary Statistics',
    version: '1.0',
    parameters: {},
    functions: [
      Stage.findAll('crud_users'),
      Stage.group(['status', 'role'], [
        { output_field: 'count', operation: 'Count' },
        { output_field: 'total_score', operation: 'Sum', input_field: 'score' },
      ]),
    ],
    tags: ['users', 'analytics', 'summary'],
  };
  
  const scriptId = await client.saveScript(script);
  console.log('✅ Script saved');
  
  const result = await client.callScript('user_summary');
  console.log(`📊 User summary (${result.records.length} groups):`);
  result.records.forEach((group) => {
    console.log(`   ${group.status?.value || group.status} ${group.role?.value || group.role}s: ${group.count?.value || group.count} users, Total Score: ${group.total_score?.value || group.total_score}`);
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
    await client.deleteCollection('crud_users');
    console.log('✅ Cleanup complete\n');
  } catch (error) {
    console.log('⚠️  Cleanup had some errors (this is okay)\n');
  }
}

async function main() {
  console.log('🚀 ekoDB CRUD Scripts Example\n');
  
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  
  try {
    await setupTestData(client);
    const scriptIds = [];
    scriptIds.push(await listUsersScript(client));
    scriptIds.push(await countByStatusScript(client));
    scriptIds.push(await avgScoreByRoleScript(client));
    scriptIds.push(await topUsersScript(client));
    scriptIds.push(await userSummaryScript(client));
    await cleanup(client, scriptIds);
    
    console.log('✅ All CRUD script examples completed!');
  } catch (error) {
    console.error('❌ Error:', error.message);
    process.exit(1);
  }
}

main();
