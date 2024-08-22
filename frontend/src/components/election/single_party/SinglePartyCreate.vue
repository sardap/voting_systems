<script setup lang="ts">
import type { GenericElectionCreate } from '@/backend'
import type { PropType } from 'vue'

defineProps({
  modelValue: {
    type: Object as PropType<GenericElectionCreate>,
    required: true
  }
})
</script>

<template>
  <div>
    <label for="num_winners">Candidate Name</label><br />
    <input v-model="modelValue.options[0]" type="text" /><br />
  </div>
</template>

<style scoped></style>
