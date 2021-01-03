import Layout from '../components/Layout'

// TODO: make this route dependent on the user id
export default function Following() {
  const USER_NAME = 'Jesse Wright'
  const people = [
    { id: 1, name: 'Frankie Fitzpatrick' },
    { id: 2, name: 'Chuck Chesterton' },
    { id: 3, name: 'Ernie MacEntire' },
    { id: 4, name: 'Robby McRobinson' },
  ]

  return (
      <Layout pageTitle={`${USER_NAME}'s Followers`}>
        <div>
          <h1 className="text-2xl">{`${USER_NAME}'s Followers`}</h1>
          <ul>
            {people.map((person) => (
              <li key={person.id}>{person.name}</li>
            ))}
          </ul>
        </div>
      </Layout>
  )
}
