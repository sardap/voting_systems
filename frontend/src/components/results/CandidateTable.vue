<script setup lang="ts">
import type { PropType } from 'vue'

defineProps({
  candidates: {
    required: true,
    type: Array as PropType<string[]>
  }
})
</script>

<template>
  <table>
    <tr>
      <th>Name</th>
    </tr>
    <tr v-for="candidate in candidates" :key="candidate">
      <td>{{ candidate }}</td>
    </tr>
  </table>
</template>

<style scoped></style>
