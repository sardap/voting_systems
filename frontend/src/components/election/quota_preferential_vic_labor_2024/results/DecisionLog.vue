<script setup lang="ts">
import type { QuotaPreferentialVicLabor2024DecisionLog } from '@/backend'
import type { PropType } from 'vue'

defineProps({
  decision_log: {
    type: Object as PropType<QuotaPreferentialVicLabor2024DecisionLog>,
    required: true
  }
})
</script>

<template>
  <div>
    <div v-for="(entry, i) in decision_log.decisions" :key="i">
      <p>#{{ i + 1 }} {{ entry }}</p>
    </div>
  </div>
</template>

<style scoped></style>
