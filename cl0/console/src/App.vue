<script setup lang="ts">
import { computed, ref } from 'vue'

interface TraceEvent { tick: number; origin: 'External' | { Internal: number }; process: number; pc: number; kind: unknown; credits_after: number }
interface RunRecord {
  type: string; id: number; revision: string; condition: string;
  initial_sha256: string; final_sha256: string; hashes_valid: boolean;
  attempt_limit: number; attempts: number; cells: number; processes: number;
  graph_objects: number; code_nodes: number; distinct_opcode_types: number;
  remaining_credits: number; world_compute_pool: number; total_compute_inventory: number;
  strict_accounting: boolean; active_processes: number;
  opcode_histogram: Record<string, number>; event_histogram: Record<string, number>;
  trace_preview?: TraceEvent[]
}
interface Contrast { initial_sha256: string; reference: string; treatment: string; delta_attempts: number; delta_cells: number; delta_code_nodes: number }
interface Analysis {
  schema: 'cl0-analysis-v1'; counts: { runs: number; matched_initials: number; invalid_hash_records: number };
  records: RunRecord[]; descriptive_contrasts: Contrast[]; inference_warning: string
}
const analysis = ref<Analysis | null>(null)
const error = ref('')
const selected = ref<RunRecord | null>(null)
const condition = ref('all')
const kinds = computed(() => ['all', ...new Set(analysis.value?.records.map(x => x.condition) ?? [])])
const filtered = computed(() => (analysis.value?.records ?? []).filter(x => condition.value === 'all' || x.condition === condition.value))
const maxAttempts = computed(() => Math.max(1, ...filtered.value.map(x => x.attempts)))
const totalAttempts = computed(() => filtered.value.reduce((acc, r) => acc + r.attempts, 0))
function isAnalysis(input: unknown): input is Analysis {
  if (!input || typeof input !== 'object') return false
  const value = input as Partial<Analysis>
  return value.schema === 'cl0-analysis-v1' && Array.isArray(value.records)
    && Array.isArray(value.descriptive_contrasts) && !!value.counts
}
async function chooseFile(event: Event) {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  error.value = ''
  try {
    if (file.size > 25_000_000) throw new Error('文件超过 25MB，请缩小分析批次')
    const parsed: unknown = JSON.parse(await file.text())
    if (!isAnalysis(parsed)) throw new Error('不支持的文件格式：需要 cl0-analysis-v1 JSON')
    analysis.value = parsed
    selected.value = parsed.records[0] ?? null
    condition.value = 'all'
  } catch (cause) {
    analysis.value = null
    selected.value = null
    error.value = cause instanceof Error ? cause.message : String(cause)
  } finally {
    input.value = ''
  }
}
const shortHash = (value: string) => value.slice(0, 12)
const eventKind = (value: unknown): string => {
  if (typeof value === 'string') return value
  if (value && typeof value === 'object') return Object.keys(value)[0] ?? 'unknown'
  return 'unknown'
}
const eventOrigin = (value: TraceEvent['origin']) =>
  value === 'External' ? '外部干预' : '内部指令'
</script>

<template>
  <main>
    <header class="masthead">
      <div class="brand"><span class="mark">C</span><div><p class="eyebrow">CYBERLIFE RESEARCH · CL-0</p><h1>实验观察台 <span>/ Observer</span></h1></div></div>
      <span class="read-only">只读观察 · 无世界写入权限</span>
    </header>
    <section class="hero">
      <div><p class="eyebrow">REPRODUCIBLE EXPERIMENTS</p>
        <h2>观察信息结构。<br /><em>不预设生命。</em></h2>
        <p>读取由 Python 分析器导出的 SQLite 实验摘要。图表描述计算、存储和因果差异；它们不是生命、适应度或开放式演化的证明。</p></div>
      <label class="filebox" for="experiment-file">
        <strong>导入分析结果</strong><span>选择 cl0-analysis-v1 JSON 文件</span><span class="upload">打开本地文件</span>
        <input id="experiment-file" type="file" accept=".json,application/json" @change="chooseFile" />
      </label>
    </section>
    <p v-if="error" class="error" role="alert">{{ error }}</p>
    <div v-if="!analysis" class="empty"><span class="empty-symbol">∅</span><h3>尚未加载实验</h3><p>先运行 Rust 实验，再执行 <code>python3 analysis/analyze.py experiment.sqlite --json report.json</code>，将结果导入此页面。</p></div>
    <template v-else>
      <section class="summary">
        <div class="metric"><p>实验记录</p><strong>{{ analysis.counts.runs }}</strong><small>SQLite persisted runs</small></div>
        <div class="metric"><p>匹配初态</p><strong>{{ analysis.counts.matched_initials }}</strong><small>SHA-256 groups</small></div>
        <div class="metric"><p>指令尝试</p><strong>{{ totalAttempts }}</strong><small>当前筛选结果</small></div>
        <div class="metric"><p>哈希异常</p><strong :class="{ warning: analysis.counts.invalid_hash_records > 0 }">{{ analysis.counts.invalid_hash_records }}</strong><small>Record consistency</small></div>
      </section>
      <div class="workbench">
        <section class="panel results">
          <div class="panel-title"><div><p class="eyebrow">01 / RUN REGISTRY</p><h3>实验记录</h3></div>
            <label>条件 <select v-model="condition"><option v-for="kind in kinds" :key="kind" :value="kind">{{ kind === 'all' ? '全部' : kind }}</option></select></label>
          </div>
          <div class="run-list">
            <button v-for="run in filtered" :key="run.type + run.id" class="run" :class="{ active: selected?.id === run.id && selected?.type === run.type }" @click="selected = run">
              <div class="run-head"><span class="run-id">{{ run.type }} / {{ run.id }}</span><span :class="run.hashes_valid ? 'ok' : 'bad'">{{ run.hashes_valid ? 'HASH OK' : 'HASH BAD' }}</span></div>
              <strong>{{ run.condition }}</strong>
              <div class="bar"><div :style="{ width: (run.attempts/maxAttempts*100) + '%' }" /></div>
              <div class="run-foot"><span>{{ run.attempts }} 次指令尝试</span><span>{{ run.cells }} 数据单元</span></div>
            </button>
            <p v-if="filtered.length === 0" class="hint">没有匹配的记录。</p>
          </div>
        </section>
        <section class="panel detail">
          <div class="panel-title"><div><p class="eyebrow">02 / STATE INSPECTOR</p><h3>终态指标</h3></div></div>
          <template v-if="selected">
            <h4>{{ selected.condition }}</h4><p class="hash">初态 SHA-256：{{ shortHash(selected.initial_sha256) }}…</p>
            <div class="details">
              <div><span>进程</span><strong>{{ selected.processes }}</strong></div><div><span>活跃进程</span><strong>{{ selected.active_processes }}</strong></div>
              <div><span>程序图节点</span><strong>{{ selected.code_nodes }}</strong></div><div><span>独立图对象</span><strong>{{ selected.graph_objects }}</strong></div>
              <div><span>共享存储单元</span><strong>{{ selected.cells }}</strong></div><div><span>剩余计算额度</span><strong>{{ selected.remaining_credits }}</strong></div>
              <div><span>世界计算池</span><strong>{{ selected.world_compute_pool ?? 0 }}</strong></div><div><span>计算总库存</span><strong>{{ selected.total_compute_inventory ?? selected.remaining_credits }}</strong></div>
            </div>
            <p class="eyebrow section-label">OPCODE DIVERSITY</p>
            <div v-for="(value, name) in selected.opcode_histogram" :key="name" class="opcode"><span>{{ name }}</span><span>{{ value }}</span></div>
            <p v-if="Object.keys(selected.opcode_histogram).length === 0" class="hint">无可执行节点。</p>
          </template>
          <p v-else class="hint">选择一条实验记录以检查其终态。</p>
        </section>
      </div>
      <section class="panel contrasts"><div class="panel-title"><div><p class="eyebrow">03 / MATCHED CONTRAST</p><h3>同初态描述性差异</h3></div></div>
        <div class="tablewrap"><table><thead><tr><th>对照条件</th><th>比较条件</th><th>Δ 指令尝试</th><th>Δ 存储单元</th><th>Δ 程序节点</th></tr></thead>
          <tbody><tr v-for="(pair, i) in analysis.descriptive_contrasts" :key="i"><td>{{ pair.reference }}</td><td>{{ pair.treatment }}</td><td>{{ pair.delta_attempts }}</td><td>{{ pair.delta_cells }}</td><td>{{ pair.delta_code_nodes }}</td></tr></tbody></table>
          <p v-if="analysis.descriptive_contrasts.length === 0" class="hint">无同初态配对数据。</p></div>
      </section>
      <section v-if="selected?.trace_preview?.length" class="panel trace">
        <div class="panel-title"><div><p class="eyebrow">04 / EVENT PROVENANCE</p><h3>事件轨迹预览</h3></div><small>最多显示 200 条</small></div>
        <div class="tablewrap"><table><thead><tr><th>逻辑序号</th><th>事件来源</th><th>进程</th><th>PC</th><th>事件</th></tr></thead>
          <tbody><tr v-for="(ev, i) in selected.trace_preview" :key="i"><td>{{ ev.tick }}</td><td>{{ eventOrigin(ev.origin) }}</td><td>{{ ev.process }}</td><td>{{ ev.pc }}</td><td>{{ eventKind(ev.kind) }}</td></tr></tbody></table></div>
      </section>
      <footer><strong>证据边界</strong><p>{{ analysis.inference_warning }}</p><p>数据在浏览器本地解析；控制台不连接 Rust 世界，不执行干预，不为对象赋予生命标签。</p></footer>
    </template>
  </main>
</template>
