<script setup lang="ts">
import { ref, type PropType } from 'vue'
import {
  from_mj_rating_to_string,
  ElectionType,
  submit_generic_vote,
  type GenericElection,
  MJRating,
  from_mj_rating_to_int
} from '@/backend'
import type { VoteOption } from '@/utils'

const props = defineProps({
  election: {
    type: Object as PropType<GenericElection>,
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
interface ThreeTwoOneOption extends VoteOption {
  rating: MJRating
}

const loading = ref<boolean>(false)
const options = ref<ThreeTwoOneOption[]>(
  props.options.map((option) => {
    return {
      ...option,
      rating: MJRating.Terrible
    }
  })
)
const ratings: MJRating[] = [
  MJRating.Terrible,
  MJRating.Poor,
  MJRating.Acceptable,
  MJRating.Good,
  MJRating.VeryGood
]

async function submit() {
  const candidates = props.election.options

  const vote: number[] = []
  for (let option_index = 0; option_index < candidates.length; option_index++) {
    const points = from_mj_rating_to_int(
      options.value.find((option) => option.index == option_index)?.rating as MJRating
    )
    vote.push(points)
  }

  loading.value = true

  const response = await submit_generic_vote(
    ElectionType.MajorityJudgment,
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

function change_rating(i: number, rating: MJRating) {
  options.value[i].rating = rating
}

function get_rating_class(i: number, rating: MJRating): string {
  if (options.value[i].rating != rating) {
    return ''
  }

  switch (rating) {
    case MJRating.VeryGood:
      return `very-good`
    case MJRating.Good:
      return `good`
    case MJRating.Acceptable:
      return `acceptable`
    case MJRating.Poor:
      return `poor`
    case MJRating.Terrible:
      return `terrible`
    default:
      return ``
  }
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
              v-for="(rating, j) in ratings"
              :key="j"
              :class="`rect-button ` + get_rating_class(i, rating)"
              @click="change_rating(i, rating)"
            >
              {{ from_mj_rating_to_string(rating) }}
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
.rect-button {
  display: inline-block;
  width: 85px;
  height: 40px;
  text-align: center;
  font-weight: bold;
  background-color: white;
  cursor: pointer;
  user-select: none;
  margin: 0 5px;
  margin-top: 3px;
  margin-bottom: 3px;
}

.rect-button:hover {
  opacity: 0.8;
}

.rect-button.very-good {
  background-color: green;
  color: #ffffff;
}

.rect-button.good {
  background-color: #4caf50;
  color: #ffffff;
}

.rect-button.acceptable {
  background-color: #ff9800;
  color: #ffffff;
}

.rect-button.poor {
  background-color: #f44336;
  color: #ffffff;
}

.rect-button.terrible {
  background-color: red;
  color: #ffffff;
}
</style>
