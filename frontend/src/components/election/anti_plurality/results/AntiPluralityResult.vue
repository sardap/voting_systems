<script setup lang="ts">
import type { GenericElection, AntiPluralityResult, GenericElectionResult } from '@/backend'
import { ref, type PropType } from 'vue'
import AntiPluralityRankingTable from './AntiPluralityRankingTable.vue'

const props = defineProps({
  election: {
    type: Object as PropType<GenericElection>,
    required: true
  },
  result: {
    type: Object as PropType<GenericElectionResult>,
    required: true
  }
})

const result = ref(props.result as AntiPluralityResult)
</script>

<template>
  <h2>Ranking table</h2>
  <AntiPluralityRankingTable :options="result.options" :result="result" />
</template>

<style scoped></style>
