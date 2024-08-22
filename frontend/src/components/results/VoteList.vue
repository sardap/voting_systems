<script setup lang="ts">
import type { ElectionResult } from '@/backend'
import type { PropType } from 'vue'

defineProps({
  election_result: {
    required: true,
    type: Object as PropType<ElectionResult>
  }
})
</script>

<template>
  <table>
    <tr>
      <th>Vote Number</th>
      <th v-for="(_, round) in election_result.candidates" :key="round">No {{ round + 1 }}</th>
    </tr>
    <tr v-for="(vote, round) in election_result.votes" :key="round">
      <td>{{ round + 1 }}</td>
      <td v-for="option_index in vote.votes" :key="option_index">
        {{ election_result.candidates[option_index] }}
      </td>
    </tr>
  </table>
</template>

<style scoped></style>
