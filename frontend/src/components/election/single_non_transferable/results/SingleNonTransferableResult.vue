<script setup lang="ts">
import type { GenericElection, SntvResult, GenericElectionResult } from '@/backend'
import { ref, type PropType } from 'vue'
import SingleNonTransferableRankingTable from './SingleNonTransferableRankingTable.vue'

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

const result = ref(props.result as SntvResult)
</script>

<template>
  <br />
  <h2>Elected count: {{ result.winners.length }}</h2>
  <br />
  <h2>Ranking table</h2>
  <SingleNonTransferableRankingTable :result="result" />
</template>

<style scoped></style>
