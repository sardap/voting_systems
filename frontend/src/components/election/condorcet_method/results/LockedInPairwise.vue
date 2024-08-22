<script setup lang="ts">
import type { CondorcetMethodResult } from '@/backend'
import { ref, type PropType, computed } from 'vue'
import * as vNG from 'v-network-graph'

const props = defineProps({
  result: {
    type: Object as PropType<CondorcetMethodResult>,
    required: true
  }
})

function get_name(index: number) {
  if (props.result.matched_pair_winner == index) {
    return props.result.options[index] + ' 🥇'
  } else {
    return props.result.options[index]
  }
}

interface LockedIn {
  from: number
  to: number[]
}

function get_locked_in(): LockedIn[] {
  if (!props.result.locked_in_pairwise_victories) {
    return []
  }

  const locked_in: LockedIn[] = []

  for (let i = 0; i < props.result.locked_in_pairwise_victories.length; i++) {
    const to: number[] = []
    for (let j = 0; j < props.result.locked_in_pairwise_victories[i].length; j++) {
      const other = props.result.locked_in_pairwise_victories[i][j]
      to.push(other)
    }
    locked_in.push({ from: i, to: to })
  }

  return locked_in
}

const locked_in = get_locked_in()

function get_nodes() {
  const locked_in = props.result.locked_in_pairwise_victories
  if (locked_in === null) {
    return {}
  }

  const nodes: Record<string, { name: string }> = {}
  for (let i = 0; i < locked_in.length; i++) {
    let name = props.result.options[i]
    nodes[`node${i + 1}`] = { name: name }
  }
  console.log(`nodes: ${JSON.stringify(nodes)}`)

  return nodes
}

function get_edges() {
  const locked_in = props.result.locked_in_pairwise_victories
  if (locked_in === null) {
    return {}
  }

  const edges: Record<string, { source: string; target: string }> = {}
  let edge_num = 0
  for (let i = 0; i < locked_in.length; i++) {
    for (let j = 0; j < locked_in[i].length; j++) {
      edges[`edge${edge_num + 1}`] = {
        source: `node${i + 1}`,
        target: `node${locked_in[i][j] + 1}`
      }
      edge_num++
    }
  }
  console.log(`edges: ${JSON.stringify(edges)}`)

  return edges
}

const nodes = get_nodes()

interface MyEdge extends vNG.Edge {
  arrow: { source: boolean; target: boolean }
}

const edges = computed(() => mergeEdges(get_edges()))

function mergeEdges(edges: vNG.Edges): Record<string, MyEdge> {
  const newEdges: Record<string, MyEdge> = {}
  Object.values(edges).forEach((edge) => {
    const pair = [edge.source, edge.target]
    const newEdgeId = pair.join(':')
    if (!(newEdgeId in newEdges)) {
      newEdges[newEdgeId] = {
        ...edge,
        arrow: { source: false, target: false }
      }
    }
    const targetSide = edge.target === pair[1] ? 'target' : 'source'
    newEdges[newEdgeId].arrow[targetSide] = true
  })
  return newEdges
}

const configs = vNG.defineConfigs({
  edge: {
    selectable: true,
    normal: {
      width: 2
    },
    type: 'straight',
    marker: {
      source: {
        type: ([edge]) => (edge.arrow.source ? 'arrow' : 'none')
      },
      target: {
        type: ([edge]) => (edge.arrow.target ? 'arrow' : 'none')
      }
    }
  }
})

function get_layout() {
  const nodes_array = Object.keys(nodes)
  const layout: Record<string, { x: number; y: number }> = {}
  for (let i = 0; i < nodes_array.length; i++) {
    layout[`node${i + 1}`] = { x: i * 100, y: i % 2 == 0 ? 0 : 200 }
  }
  return {
    nodes: layout
  }
}

const layouts = ref(get_layout())
</script>

<template>
  <div>
    <table>
      <tr>
        <th>Source</th>
        <th>Connections</th>
      </tr>
      <tr v-for="(lock, i) in locked_in" :key="i">
        <td>{{ get_name(lock.from) }}</td>
        <td>
          <div
            v-if="lock.to.length > 0"
            v-html="lock.to.map((i) => get_name(i)).join('<br/>')"
          ></div>
          <div v-else>
            <p>None</p>
          </div>
        </td>
      </tr>
    </table>
    <br />
    <div>
      <v-network-graph
        class="graph"
        :nodes="nodes"
        :edges="edges"
        v-model:layouts="layouts"
        :configs="configs"
      />
    </div>
  </div>
</template>

<style scoped>
.graph {
  max-width: 90%;
  width: 800px;
  height: 600px;
  border: 1px solid #000;
  background-color: white;
}

.runoff {
  background-color: #74ef97;
}
</style>
