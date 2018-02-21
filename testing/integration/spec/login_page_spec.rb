describe "the login page", type: :feature do
  before :each do
    visit '/login'
  end

  it "says you need to login" do
    expect(page).to have_content 'You need to login to access this page'
  end

  it "has a username field" do
    expect(page).to have_field('Username')
  end

  it "has a password field" do
    expect(page).to have_field('Password')
  end

  it "has a masked password field" do
    expect(page).to have_field('Password', type: 'password')
  end

  it "has a login button" do
    expect(page).to have_button 'Login'
  end
end
