<script setup lang="ts">
import type { GenericElection, ApprovalResult, GenericElectionResult } from '@/backend'
import { ref, type PropType } from 'vue'
import ApprovalRankingTable from './ApprovalRankingTable.vue'

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

const result = ref(props.result as ApprovalResult)
</script>

<template>
  <ApprovalRankingTable :options="result.options" :result="result" />
</template>

<style scoped></style>
