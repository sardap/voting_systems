<script setup lang="ts">
import { ref, type PropType } from 'vue'
import { ElectionType, submit_generic_vote, type ScoreElection } from '@/backend'
import type { VoteOption } from '@/utils'

const props = defineProps({
  election: {
    type: Object as PropType<ScoreElection>,
    required: true
  },
  options: {
    type: Array as PropType<VoteOption[]>,
    required: true
  },
  vote_token: {
    type: String,
    required: false
  }
})

const emits = defineEmits({
  complete: () => true,
  error: (error: string) => true
})

const loading = ref<boolean>(false)
const options = ref(
  props.options.map((option) => {
    return {
      ...option,
      points: 0
    }
  })
)

async function submit() {
  const candidates = props.election.options

  const vote: number[] = []
  for (let option_index = 0; option_index < candidates.length; option_index++) {
    const points = options.value.find((option) => option.index == option_index)?.points as number
    vote.push(points)
  }

  loading.value = true

  const response = await submit_generic_vote(
    ElectionType.Score,
    props.election.id,
    props.vote_token,
    vote
  )
  const data = await response.text()
  loading.value = false
  if (!response.ok) {
    emits(`error`, data)
    console.log(`ERROR: ${data}`)
    return
  }

  emits(`complete`)
}
</script>

<template>
  <div>
    <div v-if="loading">
      <p>Loading...</p>
    </div>
    <div>
      <div class="table">
        <div class="row header">
          <div class="col">
            <p>Name</p>
          </div>
          <div class="col">
            <p>Rating</p>
          </div>
        </div>
        <div v-for="(option, i) in options" :key="option.index" class="row">
          <div class="col">
            <p>{{ option.name }}</p>
          </div>
          <div class="col">
            <button
              v-for="(score, j) in election.max_score"
              :key="j"
              @click="options[i].points = score - 1"
              :class="`circle-button ` + (options[i].points == score - 1 ? `selected` : ``)"
            >
              {{ score - 1 }}
            </button>
          </div>
        </div>
      </div>
    </div>
    <button class="vote-button" @click="submit">🗳️VOTE🗳️</button>
  </div>
</template>

<style scoped>
@import '../../../assets/voting_table.css';

/* General styles for the circle buttons */
.circle-button {
  display: inline-block;
  width: 30px;
  height: 30px;
  text-align: center;
  border-radius: 50%;
  font-weight: bold;
  background-color: white;
  cursor: pointer;
  user-select: none;
  margin: 0 5px;
  margin-top: 3px;
  margin-bottom: 3px;
}

.circle-button:hover {
  opacity: 0.8;
}

.circle-button.selected {
  background-color: #f44336;
  color: #ffffff;
}
</style>
