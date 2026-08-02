function href(route) {
  window.location.href = route
}

function confirm_password(evt) {
  let password = document.getElementsByName('password')[0]
  let repeatPassword = document.getElementsByName('repeat_password')[0]

  if (password.value != repeatPassword.value) {
    evt.preventDefault()

    let msg = document.getElementsByClassName('msg_password')[0]
    msg.innerHTML = 'A senha e a confirmação de senha não são iguais.'
    msg.style.display = 'block'
  }
}

function show_register(show) {
  document.getElementById('register-asset').style.display = show ? 'flex' : 'none'
}

function show_edit_asset(id, name, unit_value) {
  document.getElementById('update-asset').style.display = 'flex'
  document.querySelector('input[name="asset_id"]').value = id
  document.querySelector('input[name="new_name"]').value = name
  document.querySelector('input[name="new_unit_value"]').value = unit_value
}

function hide_edit_asset() {
  document.getElementById('update-asset').style.display = 'none'
}

async function update_asset() {
  let asset_id = parseInt(document.querySelector('input[name="asset_id"]').value)
  let new_name = document.querySelector('input[name="new_name"]').value
  let new_unit_value = parseFloat(document.querySelector('input[name="new_unit_value"]').value)

  await fetch('http://localhost:3000/manage', {
    method: 'PATCH',
    headers: {'Content-Type': 'application/json'},
    body: JSON.stringify({
      asset_id,
      new_name,
      new_unit_value
    })
  }).then(() => {
    window.location.reload()
  })
}

async function delete_asset(id) {
  await fetch(`http://localhost:3000/manage/${id}`, {
    method: 'DELETE'
  }).then(() => {
    window.location.reload()
  })
}
