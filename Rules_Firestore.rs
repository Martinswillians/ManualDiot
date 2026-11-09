rules_version = '2';
service cloud.firestore {
  match /databases/{database}/documents {
    match /{col}/{id} {
      allow read: if col in ['comunicados', 'links', 'tipos'];
      allow write: if request.auth != null
                   && request.auth.token.email == 'willbrasilia@gmail.com'
                   && col in ['comunicados', 'links', 'tipos'];
    }
  }
}