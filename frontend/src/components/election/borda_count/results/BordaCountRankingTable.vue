<script setup lang="ts">
import type { BordaCountTally } from '@/backend'
import type { PropType } from 'vue'

defineProps({
  options: {
    type: Object as PropType<string[]>,
    required: true
  },
  borda_count_tally: {
    type: Object as PropType<BordaCountTally[]>,
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
        <th>Rank</th>
      </tr>
      <tr v-for="(tally, i) in borda_count_tally" :key="i">
        <td>{{ options[tally.option_index] }}</td>
        <td>{{ tally.vote_count }}</td>
        <td>{{ i + 1 }}</td>
      </tr>
    </table>
  </div>
</template>

<style scoped></style>
