<script setup lang="ts">
import type { ThreeTwoOneResult } from '@/backend'
import type { PropType } from 'vue'

defineProps({
  result: {
    type: Object as PropType<ThreeTwoOneResult>,
    required: true
  }
})
</script>

<template>
  <div>
    <table>
      <tr>
        <th>Name</th>
        <th>Finalist</th>
        <th>😞Bad</th>
      </tr>
      <tr v-for="option_index in result.semifinalists" :key="option_index">
        <td>{{ result.options[option_index] }}</td>
        <td>{{ result.finalists.includes(option_index) ? '✅' : '❌' }}</td>
        <td>{{ result.points_tally.find((j) => j.option_index == option_index)?.bad_count }}</td>
      </tr>
    </table>
  </div>
</template>

<style scoped>
.runoff {
  background-color: #74ef97;
}
</style>
