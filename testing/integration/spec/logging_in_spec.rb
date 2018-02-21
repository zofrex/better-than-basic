describe "the login process", type: :feature do
  before :each do
    visit '/login'
  end

  it "requires a username" do
    click_button 'Login'
    expect(page).to have_content 'You must enter a username'
  end

  it "requires a password" do
    click_button 'Login'
    expect(page).to have_content 'You must enter a password'
  end

  it "requires a password to check a username" do
    fill_in 'Username', with: 'bielefeld'
    click_button 'Login'
    expect(page).to have_content 'You must enter a password'
  end

  it "tells you if the user doesn't exist" do
    fill_in 'Username', with: 'bielefeld'
    fill_in 'Password', with: 'password'

    click_button 'Login'

    expect(page).to have_content 'Could not find a user with that username'
  end
end
