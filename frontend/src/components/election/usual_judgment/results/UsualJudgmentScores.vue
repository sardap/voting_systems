<script setup lang="ts">
import { rank_to_emoji } from '@/utils'
import type { PropType } from 'vue'

defineProps({
  scores: {
    type: Object as PropType<{ name: string; score: number }[]>,
    required: true
  }
})
</script>

<template>
  <div>
    <table>
      <tr>
        <th>Name</th>
        <th>Score</th>
      </tr>
      <tr v-for="(entry, i) in scores" :key="i">
        <td>{{ entry.name }} {{ i == 0 ? rank_to_emoji(i + 1) : `` }}</td>
        <td>{{ entry.score.toFixed(5) }}</td>
      </tr>
    </table>
  </div>
</template>

<style scoped></style>
