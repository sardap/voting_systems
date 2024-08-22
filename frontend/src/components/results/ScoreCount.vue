<script setup lang="ts">
import type { PropType } from 'vue'

defineProps({
  options: {
    required: true,
    type: Array as PropType<string[]>
  },
  counts: {
    required: true,
    type: Object as PropType<{ option_index: number; vote_count: number }[]>
  },
  winner: {
    required: false,
    type: Number
  }
})
</script>

<template>
  <table>
    <tr>
      <th>Name</th>
      <th>Points</th>
    </tr>
    <tr v-for="count in counts" :key="count.option_index">
      <th>{{ options[count.option_index] }}{{ count.option_index == winner ? `🥇` : `` }}</th>
      <td>{{ count.vote_count }}</td>
    </tr>
  </table>
</template>

<style scoped></style>
