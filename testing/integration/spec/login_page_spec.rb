describe "the login page", type: :feature do
  it "says login" do
    visit '/login'
    expect(page).to have_content 'Login'
  end
end
