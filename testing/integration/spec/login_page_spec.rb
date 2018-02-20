describe "the login page", type: :feature do
  it "says login" do
    visit '/'
    expect(page).to have_content 'Login'
  end
end
