import Layout from '../components/Layout'
import StyledLink from '../components/StyledLink'
// TODO: make this route dependent on the user id
export default function Following() {
  const people = [
    { id: 1, name: 'Frankie Fitzpatrick' },
    { id: 2, name: 'Chuck Chesterton' },
    { id: 3, name: 'Ernie MacEntire' },
    { id: 4, name: 'Robby McRobinson' },
    { id: 5, name: 'Stevie Stevenson' },
    { id: 6, name: 'Davy Davidson' },
    { id: 7, name: 'Earnest Kensington IV' },
  ]
  const USER_NAME = 'Jesse Wright'
  return (
    <Layout pageTitle={`${USER_NAME} following`}>
      <h1 className="text-xl mb-4">
        <span className="font-bold">{USER_NAME}</span> is{' '}
        <span className="font-bold">following</span>
      </h1>
      <ul className="">
        {people.map((person) => (
          // TODO: show icons
          <li key={person.id} className="my-2 font-medium">
            <StyledLink title={person.name} href={`/profile`} />
          </li>
        ))}
      </ul>
    </Layout>
  )
}
